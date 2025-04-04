# For Windows:
maturin build --release --compatibility off

# For Linux:
# maturin build --release --compatibility manylinux2014

source .venv/Scripts/activate
pip install target/wheels/date_type_detector-*.whl