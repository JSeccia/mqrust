from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()


class StockOpening(db.Model):
    __tablename__ = 'stock_openings'

    id = db.Column(db.Integer, primary_key=True)
    opening = db.Column(db.Float, nullable=False)

    def __repr__(self):
        return f'<StockOpening {self.id} {self.opening}>'
