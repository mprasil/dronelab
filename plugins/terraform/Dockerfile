FROM jmccann/drone-terraform:1 as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="jmccann/drone-terraform:1"
ENV DRONELAB_UPSTREAM_AUTHOR="jmccann"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/jmccann/drone-terraform"
ENV DRONELAB="/bin/drone-terraform"
