import argparse
import requests
from lxml import html
import json

# Create an ArgumentParser object
parser = argparse.ArgumentParser(description="Fetch USCIS case status")

# Add an argument for the case number
parser.add_argument("case_number", nargs="?", default="WAC2390058528", help="USCIS case number")

# Parse the command-line arguments
args = parser.parse_args()

# Use the provided or default case number
case_number = args.case_number

url = f'https://www.casestatusext.com/cases/{case_number}'

# Send a GET request to the URL
response = requests.get(url)

# Check if the request was successful (status code 200)
if response.status_code == 200:
    # Parse the HTML content of the page
    tree = html.fromstring(response.content)

    # Define the XPath expressions for case history
    xpath_case_history = "/html/body/div[3]/div/div/div[2]/section[2]/div/ul/li"


    # Find the elements using the XPath expression
    case_history_elements = tree.xpath(xpath_case_history)

    # Extract case history information
    case_history = []
    for element in case_history_elements:
        date = element.xpath(".//div[@class='ant-timeline-item-label']/text()")[0].strip()
        status = element.xpath(".//div[@class='ant-timeline-item-content']/text()")[0].strip()

        case_history.append({
            "date": date,
            "status": status,
        })

    # Define the XPath expressions for other information
    xpath_status = "/html/body/div[3]/div/div/div[2]/section[1]/div/div/div[2]/table/tbody/tr[2]/td[2]/span/span/span[2]"
    xpath_detail = "/html/body/div[3]/div/div/div[2]/section[1]/div/div/div[2]/table/tbody/tr[3]/td/span"
    xpath_case_id = "/html/body/div[3]/div/div/div[2]/section[1]/div/div/div[2]/table/tbody/tr[1]/td[1]/span"
    xpath_form = "/html/body/div[3]/div/div/div[2]/section[1]/div/div/div[2]/table/tbody/tr[1]/td[3]/span/a"

    # Find the elements using the XPath expressions
    case_status_element = tree.xpath(xpath_status)
    detail_element = tree.xpath(xpath_detail)
    form_element = tree.xpath(xpath_form)
    case_id_element = tree.xpath(xpath_case_id)

    # Extract the text content of the elements
    latest_case_status = case_status_element[0].text.strip() if case_status_element else "Not found"
    details = detail_element[0].text.strip() if detail_element else "Not found"
    form = form_element[0].text.strip() if form_element else "Not found"
    case_id = case_id_element[0].text.strip() if case_id_element else "Not found"

    # Create the final result dictionary
    result = {
        "case_id": case_id,
        "latest_case_status": latest_case_status,
        "details": details,
        "form": form,
        "case_history": case_history,
    }

    # Convert the dictionary to a JSON string and print it
    print(json.dumps(result, indent=2))
else:
    print("Failed to retrieve the webpage. Status code:", response.status_code)
