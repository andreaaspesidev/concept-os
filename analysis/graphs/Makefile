build_env:
	python3 -m venv venv
	chmod +x venv/bin/activate
	venv/bin/activate && ( \
       pip install -r requirements.txt; \
    )

build_graphs:
	venv/bin/activate && ( \
       python syscall_timings.py; \
    )	
