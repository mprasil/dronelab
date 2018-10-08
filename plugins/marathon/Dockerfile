FROM e20co/drone-marathon as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="e20co/drone-marathon"
ENV DRONELAB_UPSTREAM_AUTHOR="dellintosh"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/expansioncap/drone-marathon"
ENV DRONELAB="python /run/manage.py"
