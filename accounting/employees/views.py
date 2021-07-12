from django.shortcuts import render,get_object_or_404
from employees.models import Position, Employee, Salary
from datetime import datetime
from pytz import timezone
import pytz

def clock_in(request,clock_id):
    employee = Employee.objects.get(clock_id=clock_id)
    ...
def clock_out(request,clock_id):
    ...

def employment_applicantion(request):
    ...
def create_new_hire(request):
    
    ...

def promote_employees(request, grade=None, employee_id=None):
    ...
def promote_individual_employee(request, grade=None, employee_id=None):
    employee = get_object_or_404(Employee, id=employee_id)

    obj = Position.objects.create(pay_rate=grade)





    
