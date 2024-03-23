import requests
import json
import hashlib


secret = "YcDgFkJjDCwr5eOlGkxUlaperl7BwoWN"
base = "http://127.0.0.1:3000" + ""

payload = requests.get(f"{base}/_challenge")
payload = payload.text.encode()
print("got payload:", payload)
h = hashlib.sha3_512()
h.update(payload)
h.update(secret.encode())
h = h.hexdigest()
print("payload + secret:", h)
nonce = requests.get(f"{base}/_challenge/{payload.decode()}/{h}")
nonce = nonce.text.encode()
print("got nonce:", nonce)
h = hashlib.sha3_512()
h.update(b'short-link-token_')
h.update(nonce)
h.update(secret.encode())
h = h.hexdigest()
print("nonce + secret:", h)
token = nonce.decode() + "_" + h
print("token:", token)

link = "喵呜"
resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "Insert", "props": {"target": "https://sbchild.top/blog/"}})
print(resp.text)

resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "Update", "props": {"enabled": False}})
print(resp.text)

resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "Update", "props": {"target": "https://sbchild.top/", "enabled": True}})
print(resp.text)

resp = requests.get(f"{base}/_/{link}", allow_redirects=False)
print("redirect to:", resp.headers['Location'])

resp = requests.get(f"{base}/_/{link}111", allow_redirects=False)
print(resp.text)

resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "View", "props": {}})
print(resp.text)

resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "Delete", "props": {}})
print(resp.text)
