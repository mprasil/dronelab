FROM lizheming/drone-wechat as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="lizheming/drone-wechat"
ENV DRONELAB_UPSTREAM_AUTHOR="lizheming"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/lizheming/drone-wechat"
ENV DRONELAB="node /wechat/index.js"
