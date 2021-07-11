from django.shortcuts import render,get_object_or_404
from employees.models import Position, Employee, Salary
def employment_applicantion(request):
    ...
def new_hire(request):
    ...

def promote_employees(request, grade=None, employee_id=None):
    ...
def promote_individual_employee(request, grade=None, employee_id=None):
    employee = get_object_or_404(Employee, id=employee_id)

    obj = Position.objects.create(pay_rate=grade)





    
