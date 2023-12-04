# case_status.py

import requests
from lxml import html
import json

url = "https://www.casestatusext.com/cases/WAC2390058528"

# Send a GET request to the URL
response = requests.get(url)

# Check if the request was successful (status code 200)
if response.status_code == 200:
    # Parse the HTML content of the page
    tree = html.fromstring(response.content)

    # Define the XPath expressions
    xpath_status = "/html/body/div[3]/div/div/div[2]/section[1]/div/div/div[2]/table/tbody/tr[2]/td[2]/span/span/span[2]"
    xpath_detail = "/html/body/div[3]/div/div/div[2]/section[1]/div/div/div[2]/table/tbody/tr[3]/td/span"

    # Find the elements using the XPath expressions
    case_status_element = tree.xpath(xpath_status)
    detail_element = tree.xpath(xpath_detail)

    # Extract the text content of the elements
    latest_case_status = case_status_element[0].text.strip() if case_status_element else "Not found"
    details = detail_element[0].text.strip() if detail_element else "Not found"

    # Create a dictionary with the case status and details
    result = {
        "latest_case_status": latest_case_status,
        "details": details
    }

    # Convert the dictionary to a JSON string and print it
    print(json.dumps(result))
else:
    print("Failed to retrieve the webpage. Status code:", response.status_code)
