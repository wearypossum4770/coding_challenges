export default class Employee {
  constructor({
    employee_id,
    date_of_birth,
    department,
    position,
    start_date,
    end_date,
    date_created,
    date_modified,
  }) {
    this.employee_id = employee_id;
    this.dateOfBirth = date_of_birth;
    this._department = department;
    this.position = position;
    this.startDate = start_date;
    this.endDate = end_date;
    this.dateCreated = date_created;
    this.dateModified = date_modified;
  }
}
