FROM robertstettner/drone-cloudformation as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="robertstettner/drone-cloudformation"
ENV DRONELAB_UPSTREAM_AUTHOR="robertstettner"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/robertstettner/drone-cloudformation"
ENV DRONELAB="/bin/sh -c node /plugin/index.js"
