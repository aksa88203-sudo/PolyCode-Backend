# Qubits and Superposition in Q#

## What is a Qubit?
A qubit is the basic unit of quantum information. Unlike a classical bit (0 or 1), a qubit can exist in multiple states at the same time.

## What is Superposition?
Superposition means a qubit can be in a combination of 0 and 1 simultaneously until it is measured.

## Key Points
- Qubits can represent both 0 and 1 together
- Measurement collapses the state into 0 or 1
- Superposition enables powerful quantum computations

## Example
```qsharp
operation SuperpositionExample() : Result {
    use q = Qubit();

    H(q); // Put qubit in superposition

    let result = M(q); // Measure qubit

    Reset(q);
    return result;
}
##Practice

Create a Q# operation that applies the Hadamard gate to a qubit and measures the result.
