# EngramAI Lite Documentation

This directory contains the documentation for EngramAI Lite, built with [MkDocs](https://www.mkdocs.org/) and the [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/) theme.

## Building the Documentation

### Prerequisites

- Python 3.7 or higher
- pip (Python package manager)

### Installation

1. Install the required dependencies:

```bash
# Install from requirements.txt
pip install -r requirements.txt
```

### Local Development

To serve the documentation locally:

```bash
# Navigate to the project root (parent of this directory)
cd ..

# Start the MkDocs development server
mkdocs serve
```

This will start a local server at http://127.0.0.1:8000/. The documentation will automatically reload when you make changes to the files.

### Building Static Site

To build the static site:

```bash
# Navigate to the project root
cd ..

# Build the documentation
mkdocs build
```

This will create a `site` directory containing the generated static website.

## Documentation Structure

```
docs/
├── about/                  # Project information
│   ├── contributing.md     # Contribution guidelines
│   ├── license.md          # License information
│   └── roadmap.md          # Development roadmap
├── design/                 # Technical design documents
│   ├── data-model.md       # Core data types and relationships
│   ├── graph-engine.md     # Graph representation and traversal
│   ├── indexing.md         # Specialized indexes for queries
│   └── storage.md          # Storage layer and persistence
├── getting-started/        # Onboarding guides
│   ├── installation.md     # Installation instructions
│   └── quickstart.md       # Quick start tutorial
├── usage/                  # Usage documentation
│   ├── api.md              # API documentation (planned)
│   ├── cli.md              # Command-line interface reference
│   └── tui.md              # Terminal UI documentation (coming soon)
├── index.md                # Home page
├── README.md               # This file
└── requirements.txt        # Python dependencies for docs
```

## Adding New Documentation

1. Create a new Markdown file in the appropriate directory
2. Update the `nav` section in `mkdocs.yml` if needed
3. Follow the existing style and formatting conventions

## Writing Style Guidelines

- Use clear, concise language
- Include code examples where appropriate
- Document both usage and internal design
- Use headings to organize content
- Include diagrams when explaining complex concepts

## Reference

- [MkDocs Documentation](https://www.mkdocs.org/)
- [Material for MkDocs Documentation](https://squidfunk.github.io/mkdocs-material/)