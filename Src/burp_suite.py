import requests

def run_burp_suite(target_url=None):
    """
    Sends a request through Burp Suite's proxy.
    Allows user to enter a target URL if not provided.
    """
    if not target_url:
        target_url = input("Enter the website URL to test (e.g., https://example.com): ").strip()

    print(f"Testing {target_url} using Burp Suite proxy...")

    proxy = {
        "http": "http://localhost:8080",
        "https": "http://localhost:8080"
    }

    headers = {"User-Agent": "Python-Burp-Client"}

    try:
        response = requests.get(target_url, proxies=proxy, headers=headers, verify=False)
        print(f"\n[+] Response from {target_url}: {response.status_code}\n")

        if "vulnerable" in response.text.lower():
            print("⚠️ Potential vulnerability detected in response!")
        else:
            print("✅ No known vulnerabilities detected.")

    except requests.exceptions.RequestException as e:
        print(f"❌ Error: {e}")
