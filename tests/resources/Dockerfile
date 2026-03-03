FROM registry.access.redhat.com/ubi9 AS ubi-micro-build
RUN mkdir -p /mnt/rootfs
RUN dnf install --installroot /mnt/rootfs jq curl \
    --releasever 9 --setopt install_weak_deps=false --nodocs -y; \
    dnf --installroot /mnt/rootfs clean all
# Remove /etc so we don't overwrite /etc/passwd (which contains the keycloak user)
RUN rm -rf /mnt/rootfs/etc

FROM quay.io/keycloak/keycloak:25.0.1

COPY --from=ubi-micro-build /mnt/rootfs /

COPY wait-for-keycloak.sh /opt/keycloak/scripts/wait-for-keycloak.sh
COPY create-sirix-users.sh /opt/keycloak/scripts/create-sirix-users.sh

USER root

RUN chmod +x /opt/keycloak/scripts/wait-for-keycloak.sh /opt/keycloak/scripts/create-sirix-users.sh

USER keycloak
