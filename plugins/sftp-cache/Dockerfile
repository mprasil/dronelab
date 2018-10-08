FROM appleboy/drone-sftp-cache as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="appleboy/drone-sftp-cache"
ENV DRONELAB_UPSTREAM_AUTHOR="appleboy"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/appleboy/drone-sftp-cache"
ENV DRONELAB="/bin/drone-sftp-cache"
