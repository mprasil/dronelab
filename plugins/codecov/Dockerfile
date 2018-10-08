FROM robertstettner/drone-codecov as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="robertstettner/drone-codecov"
ENV DRONELAB_UPSTREAM_AUTHOR="robertstettner"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/robertstettner/drone-codecov"
ENV DRONELAB="entrypoint.sh"
