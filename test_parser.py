#!/usr/bin/env python3
"""
Test script to verify XML structure and demonstrate data processing
for the eSubpoena Tolls Tool
"""

import xml.etree.ElementTree as ET
import re
from datetime import datetime
import json

def normalize_phone_number(number):
    """Normalize phone number to 10 digits"""
    # Remove all non-digit characters
    digits_only = re.sub(r'[^\d]', '', str(number))
    
    # If it's a 10-digit number, return as is
    if len(digits_only) == 10:
        return digits_only
    
    # If it's 11 digits and starts with 1, remove the 1
    if len(digits_only) == 11 and digits_only.startswith('1'):
        return digits_only[1:]
    
    # If it's longer than 10 digits, take the last 10
    if len(digits_only) > 10:
        return digits_only[-10:]
    
    # Otherwise, pad with zeros to make it 10 digits
    return digits_only.zfill(10)

def parse_xml_file(filename):
    """Parse the XML file and extract call records"""
    print(f"Parsing XML file: {filename}")
    
    try:
        tree = ET.parse(filename)
        root = tree.getroot()
        
        # Find all results elements
        results = root.findall('.//results')
        print(f"Found {len(results)} call records")
        
        call_records = []
        
        for i, result in enumerate(results[:10]):  # Process first 10 for demo
            try:
                direction = result.find('messageDirection').text
                remote_number = result.find('remoteNumber').text
                start_time = result.find('startTime').text
                end_time = result.find('endTime').text
                duration = int(result.find('lengthOfCall').text)
                
                # Normalize phone number
                normalized_number = normalize_phone_number(remote_number)
                
                # Parse datetime
                start_dt = datetime.fromisoformat(start_time.replace('Z', '+00:00'))
                end_dt = datetime.fromisoformat(end_time.replace('Z', '+00:00'))
                
                record = {
                    'direction': direction,
                    'remote_number': remote_number,
                    'normalized_number': normalized_number,
                    'start_time': start_dt.isoformat(),
                    'end_time': end_dt.isoformat(),
                    'duration_seconds': duration,
                    'duration_minutes': duration / 60.0,
                    'date': start_dt.strftime('%Y-%m-%d'),
                    'time': start_dt.strftime('%H:%M:%S'),
                    'day_of_week': start_dt.strftime('%A')
                }
                
                call_records.append(record)
                
            except Exception as e:
                print(f"Error processing record {i}: {e}")
                continue
        
        return call_records
        
    except Exception as e:
        print(f"Error parsing XML file: {e}")
        return []

def generate_analytics(records):
    """Generate basic analytics from call records"""
    if not records:
        return {}
    
    total_calls = len(records)
    incoming_calls = sum(1 for r in records if r['direction'].lower() == 'incoming')
    outgoing_calls = total_calls - incoming_calls
    
    total_duration = sum(r['duration_minutes'] for r in records)
    avg_duration = total_duration / total_calls if total_calls > 0 else 0
    
    unique_numbers = len(set(r['normalized_number'] for r in records))
    
    # Count calls by number
    number_counts = {}
    for record in records:
        number = record['normalized_number']
        number_counts[number] = number_counts.get(number, 0) + 1
    
    most_frequent = sorted(number_counts.items(), key=lambda x: x[1], reverse=True)[:5]
    
    return {
        'total_calls': total_calls,
        'incoming_calls': incoming_calls,
        'outgoing_calls': outgoing_calls,
        'total_duration_minutes': total_duration,
        'average_duration_minutes': avg_duration,
        'unique_numbers': unique_numbers,
        'most_frequent_numbers': most_frequent
    }

def main():
    """Main function to test XML parsing"""
    print("=" * 60)
    print("eSubpoena Tolls Tool - XML Parser Test")
    print("=" * 60)
    print()
    
    # Parse the XML file
    records = parse_xml_file('LDS-101_item2_shipment1.xml')
    
    if not records:
        print("No records found or error occurred")
        return
    
    print(f"\nSuccessfully parsed {len(records)} call records")
    print("\nSample records:")
    print("-" * 80)
    
    for i, record in enumerate(records[:3]):
        print(f"Record {i+1}:")
        print(f"  Direction: {record['direction']}")
        print(f"  Remote Number: {record['remote_number']} -> {record['normalized_number']}")
        print(f"  Date/Time: {record['date']} {record['time']} ({record['day_of_week']})")
        print(f"  Duration: {record['duration_seconds']}s ({record['duration_minutes']:.2f} min)")
        print()
    
    # Generate analytics
    analytics = generate_analytics(records)
    
    print("Analytics Summary:")
    print("-" * 40)
    print(f"Total Calls: {analytics['total_calls']}")
    print(f"Incoming: {analytics['incoming_calls']}")
    print(f"Outgoing: {analytics['outgoing_calls']}")
    print(f"Unique Numbers: {analytics['unique_numbers']}")
    print(f"Total Duration: {analytics['total_duration_minutes']:.2f} minutes")
    print(f"Average Duration: {analytics['average_duration_minutes']:.2f} minutes")
    
    print("\nMost Frequent Numbers:")
    for number, count in analytics['most_frequent_numbers']:
        print(f"  {number}: {count} calls")
    
    print("\n" + "=" * 60)
    print("Test completed successfully!")
    print("The Rust application will provide similar functionality")
    print("with a professional GUI and Excel export capabilities.")
    print("=" * 60)

if __name__ == "__main__":
    main() 