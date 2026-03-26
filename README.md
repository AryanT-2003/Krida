# Krida (क्रीडा)

**A High-Performance Evolutionary Game Theory Framework in Rust**

A journey through Evolutionary Game Theory, powered by Rust.

**Krida** is a specialized simulation engine designed to model and analyze the dynamics of the **Iterated Prisoner’s Dilemma (IPD)**. Built with a focus on modularity and computational efficiency, it serves as a platform for studying **Evolutionary Stable Strategies (ESS)** and population genetics through the lens of modern systems programming.

---

# 🔬 Scientific Inspiration

The project is rooted in the seminal work of Robert **Axelrod’s Evolutionary Tournaments**. It examines how cooperation can emerge and persist in environments where agents act out of self-interest. Krida expands this research by introducing biological variables, including **mutation rates**, **generational resource allocation**, and **environmental noise**.

---

# 🛠 The Learning Journey: From Zero to Systems Pro

This repository serves as a living document of my growth in software development. Each milestone represents a leap in technical complexity:

## Phase 1: The Rust Foundations (Current)

- Ownership & Borrowing: Managing data without a Garbage Collector.

- Traits & Polymorphism: Defining a Strategy trait so new behaviors can be "plugged in."

- Workspace Management: Organizing a CLI and multiple Libraries (Lib) into a single Cargo workspace.

## Phase 2: Advanced Software Patterns

- Error Handling: Moving from unwrap() to robust, idiomatic error types.

- Generics: Making the simulation engine agnostic of the specific game being played.

- Unit Testing: Implementing rigorous test suites for game logic to ensure 100% deterministic results.

## Phase 3: High Performance & Scaling

- Parallisation (Rayon/Tokio): Shifting from single-threaded loops to multi-core processing for massive population sizes.

- Memory Optimization: Reducing the heap footprint of thousands of competing agents.

- Data Visualization: Exporting simulation results to JSON/CSV for external analysis.

---

# 🏗 Project Structure

```Plaintext

Krida/
├── krida-cli/          # The Command Line Interface (User Interaction)
├── krida-core/         # Core Simulation Logic (The "Brain")
└── krida-strategy/     # Implementation of various Game Theory behaviors

```
