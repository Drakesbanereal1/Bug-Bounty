import subprocess
import os
import psutil
import time

# ✅ Update paths for your system
GHIDRA_PATH = r"C:\Users\steve\Desktop\ghidra\ghidraRun.bat"  # Corrected Ghidra path
GHIDRA_SCRIPT_PATH = r"C:\Users\steve\Desktop\ghidra\Scripts\analyze.py"  # Your Ghidra script

def is_ghidra_running():
    """Checks if Ghidra is currently running to prevent multiple instances."""
    for proc in psutil.process_iter(['pid', 'name']):
        if "ghidra" in proc.info['name'].lower():
            return True
    return False

def run_ghidra():
    """Launches Ghidra only if it is not already running."""
    if not os.path.exists(GHIDRA_PATH):
        print(f"❌ Ghidra executable not found at {GHIDRA_PATH}")
        return

    if is_ghidra_running():
        print("⚠️ Ghidra is already running. Not launching a new instance.")
        return

    try:
        print("🚀 Launching Ghidra...")
        subprocess.Popen([GHIDRA_PATH], shell=True)
        time.sleep(5)  # Give it time to start
        print("✅ Ghidra launched successfully.")
    except Exception as e:
        print(f"❌ Failed to start Ghidra: {e}")

def close_ghidra():
    """Attempts to close Ghidra if it is running."""
    ghidra_found = False

    for proc in psutil.process_iter(['pid', 'name']):
        if "ghidra" in proc.info['name'].lower():
            ghidra_found = True
            try:
                print("⚠️ Closing Ghidra...")
                proc.terminate()
                proc.wait(timeout=10)
                print("✅ Ghidra closed successfully.")
            except Exception as e:
                print(f"❌ Failed to close Ghidra: {e}")

    if not ghidra_found:
        print("✅ Ghidra is not running.")

def run_ghidra_script(target_binary):
    """Runs a Ghidra script on the selected binary file."""
    if not os.path.exists(GHIDRA_PATH):
        print(f"❌ Ghidra executable not found at {GHIDRA_PATH}")
        return

    if not os.path.exists(target_binary):
        print(f"❌ Target binary file not found: {target_binary}")
        return

    if not os.path.exists(GHIDRA_SCRIPT_PATH):
        print(f"❌ Ghidra script file not found: {GHIDRA_SCRIPT_PATH}")
        return

    try:
        print(f"🔍 Running Ghidra script on {target_binary}...")

        ghidra_command = [
            GHIDRA_PATH,  # Path to Ghidra
            "-import", target_binary,  # Open the target binary
            "-scriptPath", GHIDRA_SCRIPT_PATH,  # Run the script
            "-noanalysis"  # Disable auto-analysis to prevent conflicts
        ]

        result = subprocess.run(ghidra_command, capture_output=True, text=True)

        print("\n✅ Ghidra Script Output:\n")
        print(result.stdout)
        
        if result.stderr:
            print("\n⚠️ Ghidra Errors:\n")
            print(result.stderr)

    except Exception as e:
        print(f"❌ Failed to run Ghidra script: {e}")

def ghidra_menu():
    """Submenu for Ghidra options."""
    while True:
        print("\n----------------------------------")
        print("🔹 Ghidra Options 🔹")
        print("1) Launch Ghidra")
        print("2) Analyze a Binary with Ghidra")
        print("3) Close Ghidra")
        print("4) Return to Main Menu")

        choice = input("Enter your choice: ").strip()

        if choice == "1":
            run_ghidra()
        elif choice == "2":
            binary_path = input("Enter the path to the binary file: ").strip().strip('"')  # Remove quotes
            run_ghidra_script(binary_path)
        elif choice == "3":
            close_ghidra()
        elif choice == "4":
            print("Returning to Main Menu...")
            break
        else:
            print("❌ Invalid option. Please enter a number between 1 and 4.")
