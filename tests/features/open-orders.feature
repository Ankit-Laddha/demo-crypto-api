Feature: Crypto API Open Orders

  @open
  Scenario: Successfully fetch open orders from Crypto API
    Given the Server API endpoint "/0/private/OpenOrders"
    When a POST request is sent to the endpoint with valid credentials
    Then the response status should be 200
    And the response should contain valid open orders information

  @w
  Scenario: Fail to fetch open orders for invalid API_KEY
    Given the Server API endpoint "/0/private/OpenOrders"
    When a POST request is sent to the endpoint with invalid api_key
    Then the response status should be 200
    And the response should contain error about invalid api_key

  @w
  Scenario: Fail to fetch open orders for invalid API_SECRET
    Given the Server API endpoint "/0/private/OpenOrders"
    When a POST request is sent to the endpoint with invalid api_secret
    Then the response status should be 200
    And the response should contain error about invalid api_secret