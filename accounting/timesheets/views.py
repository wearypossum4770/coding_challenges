from django.shortcuts import render

from timesheets.models import Timesheet


def timesheet_list(request):
    array = Timesheet.objects.all()
    return array
