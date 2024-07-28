Feature: Ticker info Retrieval

  @smoke
  Scenario: Successfully fetch XBT/USD trading pair information from Crypto API
    Given the Server API endpoint "/0/public/Ticker?pair=XXBTZUSD"
    When I send a GET request to the endpoint
    Then the response status should be 200
    And the response should contain "XXBTZUSD" trading pair information