import requests
import json
import hashlib


secret = "YcDgFkJjDCwr5eOlGkxUlaperl7BwoWN"
base = "http://127.0.0.1:3000" + ""

# --- Authentication ---

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

nonce = nonce.decode()
hs = h
token = nonce + "_" + hs
print("token:", token)

# --- use the Admin API ---

link = "nya"  # the name of a short-link

# add a short-link
resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "Insert", "props": {"target": "https://sbchild.top/blog/"}})
print(resp.text)

# update it (disable)
resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "Update", "props": {"enabled": False}})
print(resp.text)

# update it (enable and change the target)
resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "Update", "props": {"target": "https://sbchild.top/", "enabled": True}})
print(resp.text)

# try it (http://127.0.0.1:3000/_/nya)
resp = requests.get(f"{base}/_/{link}", allow_redirects=False)
print("redirect to:", resp.headers['Location'])

# try it (http://127.0.0.1:3000/_/nya111), and it should fail
resp = requests.get(f"{base}/_/{link}111", allow_redirects=False)
print(resp.text)

# view the status
resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "View", "props": {}})
print(resp.text)

# delete this short-link
resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "Delete", "props": {}})
print(resp.text)

# revoke this nonce, then the secret is no longer valid
resp = requests.get(f"{base}/_challenge_revoke/{nonce}/{hs}")
print("the nonce", resp.text, "is revoked")

# view the status, and it should fail
resp = requests.post(f"{base}/_update/{token}/{link}", json={"action": "View", "props": {}})
print(resp.text)
