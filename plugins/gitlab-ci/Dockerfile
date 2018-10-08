FROM appleboy/drone-gitlab-ci as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="appleboy/drone-gitlab-ci"
ENV DRONELAB_UPSTREAM_AUTHOR="appleboy"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/appleboy/drone-gitlab-ci"
ENV DRONELAB="/drone-gitlab-ci"
