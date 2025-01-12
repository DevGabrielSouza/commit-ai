# Commit-AI

**Commit-AI** is a CLI tool developed in Rust that uses the OpenAI API to automatically generate commit messages in the **Conventional Commits** format. It analyzes the changes made in the repository using `git diff` and suggests messages based on the context of the changes.

## 🚀 Features

- Generates automatic and standardized commit messages.
- Integration with Git for change analysis (`git diff`).
- Compatible with the **Conventional Commits** standard.
- Based on the OpenAI API to generate intelligent messages.

## 🛠️ Installation

1. Make sure you have **Rust** and **Cargo** installed:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:

   ```bash
   git clone https://github.com/your-username/commit-ai.git
   cd commit-ai
   ```

3. Build and install the binary:

   ```bash
   cargo build --release
   sudo mv target/release/commit-ai /usr/local/bin/
   ```

4. Set your OpenAI API key as an environment variable:

   ```bash
   export OPENAI_API_KEY="your_api_key"
   ```

   Or create a `.env` file in the same directory:

   ```env
   OPENAI_API_KEY=your_api_key
   ```

## 📋 How to Use

1. Navigate to a Git repository.
2. Run the command:
   ```bash
   commit-ai
   ```
3. `commit-ai` will analyze the changes and suggest a commit message.

## ⚙️ Advanced Configuration

### Change the OpenAI Model

By default, `commit-ai` uses the `gpt-3.5-turbo` model. You can change this in the code in `src/openai.rs` if necessary.

### Customize the Prompt

If you want to customize the prompt sent to OpenAI, edit the `create_request_body` function in the `src/openai.rs` file.

## 🛡️ Security

Make sure your API key is protected and do not expose it in public repositories.

## 🛠️ Contributing

Contributions are welcome! To get started:

1. Fork this repository.
2. Create a branch for your feature:
   ```bash
   git checkout -b feature/new-feature
   ```
3. Submit a pull request.

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

## 🙌 Acknowledgements

This project was inspired by the need to automate and standardize commits, saving time and promoting good practices in software development.
