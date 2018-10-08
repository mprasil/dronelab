FROM omerxx/drone-lambda-plugin as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="omerxx/drone-lambda-plugin"
ENV DRONELAB_UPSTREAM_AUTHOR="devops-israel"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/omerxx/drone-lambda-plugin"
ENV DRONELAB="/bin/main"
