from z3 import Solver, Int

# Create a solver instance
solver = Solver()

# Define variables
x = Int('x')
y = Int('y')

# Add some constraints
solver.add(x + y == 10)
solver.add(x - y == 4)

# Print solver state
print("Solver state before checking:")
print(solver)

# Check if the constraints have a solution
result = solver.check()
print("Solver check result:", result)

# If solution exists, print it
if result == "sat":
    model = solver.model()
    print("Solution found:")
    for var in model:
        print(f"{var} = {model[var]}")
else:
    print("No solution found.")
