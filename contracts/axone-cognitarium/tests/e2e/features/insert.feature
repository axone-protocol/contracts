Feature: Cognitarium insertion

  Scenario: Inserting some rdf data
    This scenario demonstrates inserting some rdf data into the Cognitarium smart contract.

    Given a smart contract instantiated with message:
      """yaml
      limits:
        max_triple_count: '10000'
        max_byte_size: '2000000'
        max_triple_byte_size: '300'
        max_query_limit: 4
        max_query_variable_count: 5
        max_insert_data_byte_size: '10000'
        max_insert_data_triple_count: '100'
      """
    When the smart contract is called with the following execute message:
    """yaml
    !insert_data
    format: turtle
    data: |
      @prefix ex: <http://example.com/stuff/1.0/> .
      @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

      ex:Alice a <http://www.w3.org/2002/07/owl#Person> ;
         ex:hasAge "30"^^xsd:integer ;
         ex:hasEmail "alice@example.com" .
    """
    Then response is successful
    Then response attributes should be:
      | action       | insert |
      | triple_count | 3      |
