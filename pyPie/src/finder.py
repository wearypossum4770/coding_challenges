from pathlib import Path
import json
from functools import reduce
def finder():
	with open("class_average.json") as file_data:
		data = json.load(file_data)
	return data
def satDown(data):
	 with open("class_average.json") as file_data:
		 json.dumps(data)
def gatter():
	data = finder()
	users = reduce(lambda key, value: key 
	return len(data.get('models'))

print(gatter())
