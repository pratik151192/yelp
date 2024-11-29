use std::sync::Arc;

use super::yelp::{
    business_service_server::BusinessService, Business, SearchRequest, SearchResponse, ViewRequest,
    ViewResponse,
};
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;
use tonic::{Request, Response, Status};

pub struct YelpBusinessService {
    postgres_conn_pool: Arc<Pool>,
}

impl YelpBusinessService {
    pub fn new(postgres_conn_pool: Arc<Pool>) -> YelpBusinessService {
        Self { postgres_conn_pool }
    }
}

#[tonic::async_trait]
impl BusinessService for YelpBusinessService {
   
    async fn search_businesses(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResponse>, Status> {
        let request = request.into_inner();

        let client = self
            .postgres_conn_pool
            .get()
            .await
            .map_err(|e| Status::internal(format!("Failed to get DB connection: {}", e)))?;

            let latitude = if request.latitude == 0.0 {
                40.7128 // Default to New York City latitude
            } else {
                request.latitude
            };
            
            let longitude = if request.longitude == 0.0 {
                -74.0060 // Default to New York City longitude
            } else {
                request.longitude
            };
        let radius: f64 = 5000.0; // Example: 5km radius

        // Convert `name` and `category` to `Option<&str>`
        let name: Option<String> = if request.name.trim().is_empty() {
            None
        } else {
            Some(request.name)
        };
        
        let category: Option<String> = if request.category.trim().is_empty() {
            None
        } else {
            Some(request.category)
        };
        let limit = request.limit;

        // Construct the SQL query
        // let query = "
        //     SELECT id, name, description, category, address,
        //            ST_Y(location::geometry) AS latitude,
        //            ST_X(location::geometry) AS longitude,
        //            avg_rating, num_rating
        //     FROM Businesses
        //     WHERE
        //         ST_DWithin(location::geography, ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography, $3)
        //         AND ($4::text IS NULL OR to_tsvector('english', name) @@ plainto_tsquery($4))
        //         AND ($5::text IS NULL OR category = $5)
        //     ORDER BY avg_rating DESC
        //     LIMIT $6;
        // ";

        let query = "SELECT id, name, description, category, address,
       ST_Y(location::geometry) AS latitude,
       ST_X(location::geometry) AS longitude,
       avg_rating, num_rating
FROM Businesses
ORDER BY avg_rating DESC";

        // Execute the query
        let rows = client
            .query(
                query,
                &[]
                // &[
                //     &longitude, // &f64
                //     &latitude,  // &f64
                //     &radius,    // &f64
                //     &name,      // &Option<&str>
                //     &category,  // &Option<&str>
                //     &limit,     // &i64
                // ],
            )
            .await
            .map_err(|e| Status::internal(format!("Query execution failed: {}", e)))?;

        // Map the results to the gRPC response
        let businesses: Vec<Business> = rows
            .iter()
            .map(|row| Business {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                category: row.get("category"),
                address: row.get("address"),
                latitude: row.get("latitude"),
                longitude: row.get("longitude"),
                avg_rating: row.get("avg_rating"),
                num_ratings: row.get("num_rating"),
            })
            .collect();

        let response = SearchResponse {
            businesses,
            next: "".to_string(), // Pagination token, if applicable
        };

        Ok(Response::new(response))
    }

    async fn view_business(
        &self,
        request: Request<ViewRequest>,
    ) -> Result<Response<ViewResponse>, Status> {
        let id = request.into_inner().id;

        // Mocked response
        let business = Business {
            id,
            name: "Mock Business".to_string(),
            description: "A description.".to_string(),
            category: "Food".to_string(),
            address: "123 Main St".to_string(),
            latitude: 40.7128,
            longitude: -74.0060,
            avg_rating: 4.5,
            num_ratings: 100,
        };

        let response = ViewResponse {
            business: Some(business),
        };
        Ok(Response::new(response))
    }
}
