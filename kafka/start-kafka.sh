#!/bin/bash

# Perform substitutions using environment variables
rm -f "$KAFKA_HOME"/config/server.properties

sed \
    -e "s|{{KAFKA_ADVERTISED_LISTENERS}}|${KAFKA_ADVERTISED_LISTENERS}|g" \
    -e "s|{{KAFKA_LISTENER_SECURITY_PROTOCOL_MAP}}|${KAFKA_LISTENER_SECURITY_PROTOCOL_MAP}|g" \
    -e "s|{{KAFKA_LISTENERS}}|${KAFKA_LISTENERS}|g" \
    -e "s|{{KAFKA_INTER_BROKER_LISTENER_NAME}}|${KAFKA_INTER_BROKER_LISTENER_NAME}|g" \
    -e "s|{{KAFKA_ZOOKEEPER_CONNECT}}|${KAFKA_ZOOKEEPER_CONNECT}|g" \
     "$KAFKA_HOME"/config/server.properties.template > "$KAFKA_HOME"/config/server.properties

# Start Kafka
exec "$KAFKA_HOME"/bin/kafka-server-start.sh "$KAFKA_HOME"/config/server.properties
