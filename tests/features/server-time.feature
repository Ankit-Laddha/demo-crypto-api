Feature: Crypto API Time Retrieval

  @smoke
  Scenario: Successfully fetch server time from Crypto API
    Given the Server API endpoint "/0/public/Time"
    When I send a GET request to the endpoint
    Then the response status should be 200
    And the response should contain valid server time