import subprocess
import os

KLEE_PATH = "klee"  # No .exe since it's in WSL
OUTPUT_DIR = "klee-out-0"
LOG_FILE = "klee_log.txt"

def check_file_exists(file_path):
    """Checks if a file exists at the given path."""
    if not os.path.exists(file_path):
        print(f"❌ Error: File not found at {file_path}")
        return False
    return True

def get_target_file():
    """Prompts the user to enter a target file path for KLEE analysis."""
    while True:
        target_file = input("🔍 Enter the path to the LLVM bitcode (.bc) file: ").strip()

        # Convert Windows path to WSL path
        target_file_wsl = target_file.replace("C:\\", "/mnt/c/").replace("\\", "/")

        if check_file_exists(target_file):
            return target_file_wsl  # Use WSL-compatible path
        else:
            print("⚠️ Invalid file. Please try again.")

def run_klee(target_file):
    """Runs KLEE on the user-specified target file using WSL."""
    print(f"🚀 Running KLEE on {target_file}...")

    klee_command = [
        "wsl",  # Run inside WSL
        KLEE_PATH,
        "--search=nurs:covnew",
        "--output-dir=" + OUTPUT_DIR,
        target_file
    ]

    try:
        result = subprocess.run(klee_command, capture_output=True, text=True)

        with open(LOG_FILE, "w") as log:
            log.write(result.stdout)
            log.write("\n--- ERRORS ---\n")
            log.write(result.stderr)

        print("✅ KLEE Execution Complete!")
        print("📄 Logs saved to:", LOG_FILE)

        if "error" in result.stderr.lower() or "error" in result.stdout.lower():
            print("⚠️ Warning: KLEE detected potential issues in execution!")

    except Exception as e:
        print(f"❌ Failed to execute KLEE: {e}")

def analyze_klee_output():
    """Parses and analyzes KLEE's output directory for discovered bugs."""
    if not os.path.exists(OUTPUT_DIR):
        print(f"⚠️ Warning: {OUTPUT_DIR} directory not found. Ensure KLEE ran successfully.")
        return

    print("\n🔍 Analyzing KLEE results for potential vulnerabilities...")
    found_issues = False

    for root, _, files in os.walk(OUTPUT_DIR):
        for file in files:
            if file.endswith(".err"):
                print(f"⚠️ Potential bug found: {file} in {root}")
                found_issues = True
            elif file.endswith(".ktest"):
                print(f"✅ Generated test case: {file} in {root}")

    if not found_issues:
        print("✅ No errors found in symbolic execution.")

def klee_menu():
    """Submenu for KLEE options."""
    while True:
        print("\n----------------------------------")
        print("🔹 KLEE Options 🔹")
        print("1) Run KLEE Analysis")
        print("2) Analyze KLEE Output")
        print("3) Return to Main Menu")

        choice = input("Enter your choice: ").strip()

        if choice == "1":
            target_file = get_target_file()
            run_klee(target_file)
        elif choice == "2":
            analyze_klee_output()
        elif choice == "3":
            print("Returning to Main Menu...")
            break
        else:
            print("❌ Invalid option. Please enter a number between 1 and 3.")

if __name__ == "__main__":
    klee_menu()
