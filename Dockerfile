FROM archlinux/base

# dependencies
RUN pacman -Suy --noconfirm rustup make gcc pkgconf sqlite \
python python-sqlalchemy yarn python-google-api-python-client python-pip
RUN pip install google_auth_oauthlib && rustup install nightly

# workdir
WORKDIR /cw

# copy to workdir
COPY . .

# build
RUN make

# expose ports
EXPOSE 8000

# run cmd
CMD ["cargo", "run", "--release"]
