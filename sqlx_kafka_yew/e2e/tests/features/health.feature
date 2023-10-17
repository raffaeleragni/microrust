Feature: Health check the system

  Scenario: Health check is OK
    Given A started system
    When Checking for health
    Then Health check was OK
