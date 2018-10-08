FROM lucap/drone-netlify as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="lucap/drone-netlify"
ENV DRONELAB_UPSTREAM_AUTHOR="lucaperret"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/lucaperret/drone-netlify"
ENV DRONELAB="/bin/sh -c /bin/script.sh"
