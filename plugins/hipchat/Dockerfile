FROM jmccann/drone-hipchat as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="jmccann/drone-hipchat"
ENV DRONELAB_UPSTREAM_AUTHOR="jmccann"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/jmccann/drone-hipchat"
ENV DRONELAB="/bin/drone-hipchat"
