#!/bin/bash
# Uso: ./scripts/network_profiles.sh [estable|latencia|perdida|cortes]

CONTAINER_NAME="couchdb-local"
CONTAINER_PID=$(docker inspect -f '{{.State.Pid}}' $CONTAINER_NAME 2>/dev/null)

if [ -z "$CONTAINER_PID" ]; then
    echo "Error: Contenedor $CONTAINER_NAME no está activo."
    exit 1
fi

PROFILE=$1

case $PROFILE in
    estable)
        echo "Aplicando red estable (sin alteraciones)..."
        sudo nsenter -t $CONTAINER_PID -n tc qdisc del dev eth0 root 2>/dev/null || true
        ;;
    latencia)
        echo "Aplicando perfil de Alta Latencia (250ms)..."
        sudo nsenter -t $CONTAINER_PID -n tc qdisc replace dev eth0 root netem delay 250ms 20ms
        ;;
    perdida)
        echo "Aplicando perfil de Pérdida de Paquetes (5%)..."
        sudo nsenter -t $CONTAINER_PID -n tc qdisc replace dev eth0 root netem loss 5%
        ;;
    cortes)
        echo "Simulando corte abrupto de conectividad (100% pérdida)..."
        sudo nsenter -t $CONTAINER_PID -n tc qdisc replace dev eth0 root netem loss 100%
        ;;
    *)
        echo "Uso: $0 {estable|latencia|perdida|cortes}"
        exit 1
        ;;
esac
echo "Perfil de red '$PROFILE' aplicado correctamente."