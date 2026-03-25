import socket
import threading
import traceback

import hdefereval
import hou

MAX_SCRIPT_SIZE = 10 * 1024 * 1024  # 10 MB
LIVE_LINK_PORT = 18080
AUTH_TOKEN = "houdini_ramen_secret_2026"


class HoudiniLiveLinkServer:
    def __init__(self, host="127.0.0.1", port=LIVE_LINK_PORT):
        self.host = host
        self.port = port
        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            self.server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            self.server_socket.bind((self.host, self.port))
        except OSError:
            self.server_socket.close()
            raise
        self._stop_event = threading.Event()

    def start(self):
        self.server_socket.listen(1)
        print(f"🍜 Houdini Ramen: Listening on {self.host}:{self.port}...")
        self.server_socket.settimeout(1.0)

        while not self._stop_event.is_set():
            try:
                client, _addr = self.server_socket.accept()
                client.settimeout(5.0)
                try:
                    self._handle_client(client)
                finally:
                    client.close()
            except socket.timeout:
                continue
            except OSError as e:
                if not self._stop_event.is_set():
                    print(f"❌ Server error: {e}")

    def _handle_client(self, client):
        chunks = []
        total = 0
        is_oversize = False

        while True:
            packet = client.recv(4096)
            if not packet:
                break
            chunks.append(packet)
            total += len(packet)
            if total > MAX_SCRIPT_SIZE:
                is_oversize = True
                break

        if is_oversize:
            print("❌ Received data exceeds maximum allowed size, dropping.")
            client.sendall(b"ERROR\nReceived data exceeds maximum allowed size.")
            return

        if not chunks:
            client.sendall(b"ERROR\nReceived empty script.")
            return

        try:
            payload = b"".join(chunks).decode("utf-8")
        except UnicodeDecodeError:
            client.sendall(b"ERROR\nInvalid UTF-8 encoding in payload.")
            return

        auth_prefix = f"AUTH:{AUTH_TOKEN}\n"
        if not payload.startswith(auth_prefix):
            print("❌ Unauthorized connection attempt rejected.")
            client.sendall(b"ERROR\nUnauthorized payload. Access denied.")
            return
        script = payload[len(auth_prefix) :]

        print("✅ Received valid script from Rust, executing...")
        response = self._execute_in_houdini(script)
        client.sendall(response)

    @staticmethod
    def _execute_in_houdini(script):
        def task():
            try:
                exec(script, {"hou": hou, "__builtins__": __builtins__})
                return b"OK"
            except Exception:
                return f"ERROR\n{traceback.format_exc()}".encode("utf-8")

        try:
            return hdefereval.executeInMainThreadWithResult(task)
        except Exception as e:
            return f"ERROR\nFailed to schedule execution: {e}".encode("utf-8")

    def stop(self):
        self._stop_event.set()
        self.server_socket.close()


if hasattr(hou.session, "ramen_server"):
    hou.session.ramen_server.stop()

try:
    server = HoudiniLiveLinkServer()
    hou.session.ramen_server = server

    thread = threading.Thread(target=server.start)
    thread.daemon = True
    thread.start()
except OSError as err:
    print(
        f"❌ Houdini Ramen: Failed to start live-link server on port {LIVE_LINK_PORT}: {err}"
    )
