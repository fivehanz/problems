

default:
	cd structures && jupyter lab


install:
	pip install jupyterlab
	cargo install evcxr_jupyter
	evcxr_jupyter --install


rtx:
	# brew install libb2 openssl readline gettext xz
	env PYTHON_CONFIGURE_OPTS="--enable-optimizations" env LDFLAGS="-fuse-ld=lld" ARCHFLAGS="-arch arm64" rtx i
