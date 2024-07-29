Feature: Crypto API Open Orders

  @open
  Scenario: Successfully fetch open orders from Kraken API
    Given the Server API endpoint "/0/private/OpenOrders"
    When a POST request is sent to the endpoint with valid credentials
    Then the response status should be 200
    And the response should contain valid open orders information