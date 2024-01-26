# Use an appropriate base image
FROM debian:bookworm-slim

# Set environment variables for Kafka version and Scala version
ENV KAFKA_VERSION=3.6.1 SCALA_VERSION=2.13 JAVA_HOME=/java/jdk-21.0.2  KAFKA_HOME=/kafka

WORKDIR ${KAFKA_HOME}
RUN mkdir /java

# Install necessary tools
RUN apt-get update -y && apt-get install -y wget netcat-traditional

# Install Java
RUN wget https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz

# Extract and list contents to determine the correct directory structure
RUN tar -xzf openjdk-21.0.2_linux-x64_bin.tar.gz \
    && mv jdk-21.0.2 /java \
    && rm openjdk-21.0.2_linux-x64_bin.tar.gz

RUN wget https://downloads.apache.org/kafka/${KAFKA_VERSION}/kafka_${SCALA_VERSION}-${KAFKA_VERSION}.tgz \
    && tar -xzf kafka_${SCALA_VERSION}-${KAFKA_VERSION}.tgz --strip-components=1 -C /kafka \
    && rm kafka_${SCALA_VERSION}-${KAFKA_VERSION}.tgz


# Set environment variables for Kafka configuration
ENV KAFKA_ADVERTISED_LISTENERS="INTERNAL://kafka:9092,EXTERNAL://localhost:9093"
ENV KAFKA_LISTENER_SECURITY_PROTOCOL_MAP="INTERNAL:PLAINTEXT,EXTERNAL:PLAINTEXT"
ENV KAFKA_LISTENERS="INTERNAL://0.0.0.0:9092,EXTERNAL://0.0.0.0:9093"
ENV KAFKA_INTER_BROKER_LISTENER_NAME="INTERNAL"
ENV KAFKA_ZOOKEEPER_CONNECT="zookeeper:2181"

COPY ./start-kafka.sh ${KAFKA_HOME}/start-kafka.sh
RUN chmod +x start-kafka.sh
# Copy the custom server.properties template
COPY server.properties.template $KAFKA_HOME/config/

ENTRYPOINT ["/bin/bash", "-c", "./start-kafka.sh"]

# Expose Kafka's internal and external ports
EXPOSE 9092 9093
