FROM vallard/drone-kube as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="vallard/drone-kube"
ENV DRONELAB_UPSTREAM_AUTHOR="vallard"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/vallard/drone-kube"
ENV DRONELAB="/bin/drone-kube"
