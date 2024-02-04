# rustle (python rle lib built with rust)

## Setup

```bash
git clone https://github.com/AineeJames/rustle.git
cd rustle
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

or

```bash
pip install rustle
```

## Example

```python
import rustle

print(rustle.encode_str('aaaabbbcc')) # -> 'a4b3c2'
print(rustle.decode_str('a4b3c2')) # -> 'aaaabbbcc'
```
