# ==============================================================================================================================================================================================================================================
# Build: docker build -t lt2330 .
# Run: docker run -d -p 8888:8888 --name lt2330 lt2330
# ==============================================================================================================================================================================================================================================

FROM rust
LABEL maintainer="Andrew Tait <Andrew.Tait@decisionmechanics.com>"

ARG PATH="/root/miniconda3/bin:${PATH}"
ENV PATH="/root/miniconda3/bin:${PATH}"

RUN apt-get update && apt-get install -y wget && rm -rf /var/lib/apt/lists/*

RUN wget \
    https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh \
    && mkdir /root/.conda \
    && bash Miniconda3-latest-Linux-x86_64.sh -b \
    && rm -f Miniconda3-latest-Linux-x86_64.sh

RUN conda install -c conda-forge jupyterlab
RUN conda install -c anaconda cmake -y

RUN cargo install evcxr_jupyter
RUN evcxr_jupyter --install

RUN conda install -c conda-forge jupyterlab-plotly-extension
RUN pip install jupyterlab_darkside_ui

EXPOSE 8888

RUN mkdir /data
WORKDIR /data

RUN mkdir /data/course
COPY course /data/course/

RUN mkdir /data/exercises
COPY exercises /data/exercises

ENTRYPOINT ["jupyter", "lab", "--ip=0.0.0.0", "--allow-root", "--NotebookApp.password='sha1:2938ab7381e2:2a9204a79cbcfd2924f77a04c0c890f690414de8'"]
