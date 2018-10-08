FROM cschlosser/drone-ftps as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="cschlosser/drone-ftps"
ENV DRONELAB_UPSTREAM_AUTHOR="christophschlosser"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/christophschlosser/drone-ftps"
ENV DRONELAB="/bin/sh -c /bin/upload.sh"
