FROM robertstettner/drone-npm-auth as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="robertstettner/drone-npm-auth"
ENV DRONELAB_UPSTREAM_AUTHOR="robertstettner"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/robertstettner/drone-npm-auth"
ENV DRONELAB=""
