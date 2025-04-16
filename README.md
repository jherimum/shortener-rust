

- all comunications between microservices will use grpc and will use a circuit break.


- Services

1 - Redirect Service: MicroService responsible for make the redirects. it will access the links service to retrieve the links and will have a cache(redis). 
2 - Links service: Microservice responsible for manage the short links. 
3 - Key Generator Service: Microservice responsible for generates the short link's keys.
4 - Statistic Service: microservice responsible for manage the link access statistics
5 - user Service: responsible for manage users, authentication
6 - Api Gateway
