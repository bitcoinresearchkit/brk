uv run pytest tests/basic.py -s
uvx pydoc-markdown > DOCS.md
uv build
uvx uv-publish
