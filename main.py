import sys
import os

# Get the directory where main.py is located
BASE_DIR = os.path.dirname(os.path.abspath(__file__))

# Add the 'Src' directory to sys.path
SRC_DIR = os.path.join(BASE_DIR, "Src")
sys.path.append(SRC_DIR)

import utils
import burp_suite
import ghidra
import klee
import z3_solver_test  # Fixed reference

def main():
    """Bug Bounty Tool Main Menu"""
    print("Bug Bounty Project Running...")
    utils.log_action("Bug Bounty tool started")

    while True:
        print("\n----------------------------------")
        print("Menu options:")
        print("1) Run Burp Suite")
        print("2) Run KLEE")
        print("3) Run Z3")
        print("4) Launch Ghidra")
        print("5) Exit (Closes Ghidra if running)")

        choice = utils.prompt_user()

        if choice == "1":
            target_url = input("Enter the website URL to test (e.g., https://example.com): ").strip()
            if target_url:
                burp_suite.run_burp_suite(target_url)
            else:
                print("❌ Invalid URL provided.")

        elif choice == "2":
            target_file = input("Enter the path to the LLVM bitcode (.bc) file: ").strip()
            if target_file and os.path.exists(target_file):
                klee.run_klee_menu()
            else:
                print("❌ Invalid or missing file path.")

        elif choice == "3":
            z3_solver_test.z3_menu()  # Calls the Z3 Menu

        elif choice == "4":
            ghidra.ghidra_menu()  # 🔹 FIX: Enter Ghidra menu instead of just launching it

        elif choice == "5":
            exit_program()
            break

        else:
            print("Invalid option. Please enter a number between 1 and 5.")

def exit_program():
    """Handles exiting the program and closing Ghidra."""
    print("Closing Ghidra (if running) and exiting...")
    ghidra.close_ghidra()
    print("Goodbye!")

if __name__ == "__main__":
    main()
