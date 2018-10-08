FROM lucap/drone-now as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="lucap/drone-now"
ENV DRONELAB_UPSTREAM_AUTHOR="lucaperret"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/lucaperret/drone-now"
ENV DRONELAB="/bin/sh -c /bin/script.sh"
