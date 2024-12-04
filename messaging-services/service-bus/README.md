# Create a Resource
You need to create a resource group before creating a Service Bus resource. Then, you can create the Service Bus resource using the following commands:
1. Creating a namespace for a Service Bus Queue.
    ```sh 
    az servicebus namespace create \
    --resource-group az204-svcbus-rg \
    --name <myNameSpaceName> \
    --location <myLocation>
    ```
2. Creating a Service Bus Queue.
    ```sh 
    az servicebus queue create --resource-group az204-svcbus-rg \
    --namespace-name <myNameSpaceName> \
    --name az204-queue
    ```
You can retrieve the connection string in the Azure Portal in Shared Access Policy under the settings section for the recently created resource. Choose the Primary Connection String.

# Installing the package necessary
1. Add the Azure.Messaging.ServiceBus package.
    ```sh
    dotnet add package Azure.Messaging.ServiceBus
    ```

# Deleting resources
```sh
az group delete --name az204-svcbus-rg --no-wait
```